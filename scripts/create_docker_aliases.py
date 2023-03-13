from util import get_all_images_names, crane_auth, crane_tag
import sys




registry = sys.argv[1]
password = sys.argv[2]
user = sys.argv[3]
project_id = 32888992
branch_name = sys.argv[4]
authorized_key = sys.argv[5]
tag_from = "master"

if (registry == "" or password == "" or user == "" or  authorized_key == ""):
    raise Exception("error, one of the variables is not set")

images = get_all_images_names(authorized_key,project_id)
crane_auth(user, password, registry)
for image in images:
    try:
        crane_tag(image, branch_name, tag_from)
    except Exception as err:
        
        print(err)
    else:
        print(f'Tagged {image}:{tag_from} with {branch_name}')