from util import get_all_images_names, crane_auth, crane_tag, check_if_docker_image_exist
import sys
from datetime import datetime



registry = sys.argv[1]
password = sys.argv[2]
user = sys.argv[3]
project_id = 32888992
branch_name = sys.argv[4]
authorized_key = sys.argv[5]
tag_from = "master"

if (registry == "" or password == "" or user == "" or  authorized_key == ""):
    raise Exception("error, one of the variables is not set")
print(datetime.now())
images = get_all_images_names(authorized_key,project_id)
print(datetime.now())
crane_auth(user, password, registry)
print(datetime.now())
for image in images:
    try:
        print(f"Checkning if {image}:{branch_name} exists...")
        check_if_docker_image_exist(f"{image}:{branch_name}")
    
    except Exception as err:
        print("It doesn't. Tagging.")
        try: 
            crane_tag(image, branch_name, tag_from)
        except Exception as err:
            print(f"Error tagging {image}:{tag_from} with {branch_name}.")
        else: 
            print(f'Tagged {image}:{tag_from} with {branch_name}.')
    else:
        print("Image exists.")