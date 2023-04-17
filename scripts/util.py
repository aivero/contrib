#!/usr/bin/env python3
import subprocess
import os
import yaml
import sys
import re
import asyncio
import pathlib
import functools
import requests 
import json


def git_init():
    # git checkout branch
    call(["git", "checkout", "-B", os.environ["CI_BUILD_REF_NAME"], os.environ["CI_BUILD_REF"]])

    # Fetch all branches
    call(["git", "config", "remote.origin.fetch", '"+refs/heads/*:refs/remotes/origin/*"'])

    # Fetch
    call(["git", "fetch", "--unshallow"])


def conan_init(repos):
    call(
        [
            "conan",
            "config",
            "install",
            os.environ["CONAN_CONFIG_URL"],
            "-sf",
            os.environ["CONAN_CONFIG_DIR"],
        ],
    show=True)
    for repo in repos:
        call(
            [
                "conan",
                "user",
                os.environ["CONAN_LOGIN_USERNAME"],
                "-p",
                os.environ["CONAN_LOGIN_PASSWORD"],
                "-r",
                repo,
            ],
        show=True)


def get_commit():
    if "CI_COMMIT_SHA" in os.environ:
        return os.environ["CI_COMMIT_SHA"]
    branch = get_branch()
    output = call(["git", "show-ref", branch, "--heads", "--tag", "-s"])
    return output[:-1]


def get_branch():
    if "CI_COMMIT_REF_NAME" in os.environ:
        return os.environ["CI_COMMIT_REF_NAME"]
    output = call(["git", "rev-parse", "--abbrev-ref", "HEAD"])[:-1]
    return output


def get_default_branch():
    if "CI_DEFAULT_BRANCH" in os.environ:
        return os.environ["CI_DEFAULT_BRANCH"]
    return "master"


def find_parent_branch():
    print("Find parent branch")

    # Get current branch
    cur_branch = get_branch()
    if cur_branch == get_default_branch():
        return None

    # Get branch data
    output = call(["git", "branch", "-a"])
    all_branches = output[:-1].split("\n")
    print(f"Raw branches: {all_branches}")
    all_branches = map(lambda l: l.strip(), all_branches)
    branches = list(
        filter(lambda l: not (l.startswith("*") or l.endswith(cur_branch)), all_branches)
    )
    print(f"Filtered branches: {branches}")

    def get_merge_base(branch):
        print(f"Comparing {branch}")
        (exit_code, output) = call(["git", "merge-base", cur_branch, branch], ret_exit_code=True)
        if exit_code != 0 or output.startswith("warning:"):
            print(f"`git merge-base {cur_branch} {branch}` Failed with output:\n{output}")
            return [99999, "00000000000", branch]

        print(f"Merge base output: {output}")
        merge_base = output[:-1].splitlines()[-1]

        output = call(["git", "rev-list", "--count", f"{cur_branch}...{merge_base}"])
        commits_to_merge_base = output[:-1]

        return [int(commits_to_merge_base), merge_base, branch]

    parent_branch = functools.reduce(
        lambda a, b: a if a[0] < b[0] else b,
        map(get_merge_base, branches),
    )[2]
    if parent_branch.startswith("remotes/origin/"):
        parent_branch = parent_branch[15:]
    print(parent_branch)
    return parent_branch


def file_contains(file, strings):
    if isinstance(strings, str):
        strings = [strings]
    with open(file, "r", encoding="utf-8") as f:
        content = f.read()
        for string in strings:
            if not string in content:
                return False
    return True


def background(f):
    def wrapped(*args, **kwargs):
        return asyncio.get_event_loop().run_in_executor(None, f, *args, **kwargs)

    return wrapped


def call(cmd, show=False, ret_exit_code=False):
    child = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    fulloutput = b""
    while True:
        output = child.stdout.readline()
        if output == b"" and child.poll() is not None:
            break
        if output:
            if show:
                print(output.decode("utf-8"), end="")
            fulloutput += output
    fulloutput = fulloutput.decode("utf-8")
    exit_code = child.poll()
    if ret_exit_code:
        return (exit_code, fulloutput)
    if exit_code != 0:
        raise RuntimeError(fulloutput)
    return fulloutput


def find_branches():
    reflog = call(["git", "reflog"])
    match = re.search("^.*from (.*) to (.*)\n", reflog)
    return (match[2], match[1])


def find_instances():
    # Get list of devops.yml files
    devops_paths = pathlib.Path(".").glob("**/devops.yml")

    # Get name of all conan package instances
    ints = set()
    for devops_path in devops_paths:
        with open(devops_path, "r") as devops_file:
            conf = yaml.safe_load(devops_file)
            if not conf:
                continue
            for instance in conf:
                # If version is in sha commit format
                if not instance or "version" not in instance:
                    if instance and "name" in instance:
                        name = instance["name"]
                    else:
                        name = os.path.basename(os.path.dirname(devops_path))
                    conanfile = os.path.join(os.path.dirname(devops_path), "conanfile.py")
                    proprietary = os.path.exists(conanfile) and file_contains(
                        conanfile, '= "Proprietary"'
                    )
                    yield (name, proprietary)


# Create alias from newest commit hash to branch
@background
def create_alias(
    ins, commit, branch, parent_branch, fetch_repo, public_repo=None, internal_repo=None
):
    name = ins[0]
    proprietary = ins[1]
    match = None
    # Find hash locally
    (exit_code, output) = call(["conan", "get", f"{name}/{parent_branch}"], ret_exit_code=True)
    if exit_code == 0:
        match = re.search('alias = ".*/(.*)"\n', output)
    # Then try finding hash from remote
    if not match:
        (exit_code, output) = call(
            ["conan", "get", "-r", fetch_repo, f"{name}/{parent_branch}"], ret_exit_code=True
        )
        if exit_code == 0:
            match = re.search('alias = ".*/(.*)"\n', output)
    if match:
        sha = match[1]
    else:
        # Fallback to HEAD commit hash
        sha = commit

    # Conan has a 50 char limit for versions
    branch = branch[:50]

    print(f"Creating alias: {name}/{branch} to {name}/{sha}")
    call(["conan", "alias", f"{name}/{branch}", f"{name}/{sha}"])

    repo = internal_repo if proprietary else public_repo
    if repo:
        print(f"Uploading alias: {name}/{branch} to {name}/{sha} to {repo}")
        call(["conan", "upload", f"{name}/{branch}", "--all", "-c", "-r", repo])


def create_aliases(commit, branch, parent_branch, fetch_repo, public_repo=None, internal_repo=None):
    for ins in find_instances():
        create_alias(ins, commit, branch, parent_branch, fetch_repo, public_repo, internal_repo)


@background
def remove_alias(ins, branch, public_repo, internal_repo):
    name = ins[0]
    proprietary = ins[1]
    repo = internal_repo if proprietary else public_repo
    print(f"Removing alias: {ins[0]}/{branch}")
    call(["conan", "remove", f"{ins[0]}/{branch}", "-f", "-r", repo])


def remove_aliases(branch, public_repo, internal_repo):
    for ins in find_instances():
        remove_alias(ins, branch, public_repo, internal_repo)
        
        

def get_all_images_names(key, project_id) -> str:
    def get_page(key, project_id, page=0):
        page_expression = "" if page == 0 else f'?page={page}' 
        response = requests.get(f'https://gitlab.com/api/v4/projects/{project_id}/registry/repositories{page_expression}', headers={"PRIVATE-TOKEN" :key })
        if (response.status_code != 200):
            print("status code != 200")
            raise Exception("response:", response.text)
        return response
    
    images = []
    response = get_page(key, project_id, 0)
    i = 1
    while response.text != "[]":
        
        response = get_page(key, project_id, i)
        parsed = json.loads(response.text)
        images = images + [i["location"] for i in parsed ]
        i = i + 1

    return images
        

    
    
        
def crane_tag(image, alias, source="master" ):
    call(["crane", "tag", f'{image}:{source}', alias])
    
def crane_auth(user, password, registry):
    call(["crane", "auth", "login", "-u", user, "-p", password, registry ])
    
def check_if_docker_image_exist(image):
    call(["crane", "manifest", image])
    
 