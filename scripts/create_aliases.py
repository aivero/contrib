import util
import os
import sys
import subprocess

# Init git and conan in CICD
if "CI" in os.environ:
    fetch_repo = os.environ["CONAN_REPO_ALL"]
    print(f"Fetching from: {fetch_repo}")
    public_repo = os.environ["CONAN_REPO_PUBLIC"]
    print(f"Uploading public to: {public_repo}")
    internal_repo = os.environ["CONAN_REPO_INTERNAL"]
    print(f"Uploading internal to: {internal_repo}")

    fetch_dev_repo = os.environ["CONAN_REPO_DEV_ALL"]
    print(f"Fetching from: {fetch_dev_repo}")
    public_dev_repo = os.environ["CONAN_REPO_DEV_PUBLIC"]
    print(f"Uploading public to: {public_dev_repo}")
    internal_dev_repo = os.environ["CONAN_REPO_DEV_INTERNAL"]
    print(f"Uploading internal to: {internal_dev_repo}")
    util.conan_init((fetch_repo, public_repo, internal_repo, fetch_dev_repo, public_dev_repo, internal_dev_repo))
else:
    print("Not uploading any packages")
    fetch_repo = "dev-all"
    public_repo = None
    internal_repo = None


branch = util.get_branch()
print(f"Branch: {branch}")
if branch == util.get_default_branch():
    print("Skipping default branch")
    sys.exit(0)
commit = util.get_commit()
print(f"Commit: {commit}")

parent_branch = None
if "PARENT_BRANCH" in os.environ:
    parent_branch = os.environ["PARENT_BRANCH"]
else:
    parent_branch = util.find_parent_branch()

print(f"Parent Branch: {parent_branch}")

util.create_aliases(
    commit,
    branch,
    parent_branch,
    fetch_repo,
    public_repo,
    internal_repo,
)
