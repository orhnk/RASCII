# https://api.github.com/repos/utfeight/Rascii/contributors?per_page=100&page=1

import requests
import os

url = "https://api.github.com/repos/utfeight/Rascii/contributors?per_page=100&page=1"
formats = ["jpeg", "jpg", "png"]
size = 30
rascii_args = f"-c -w {size}"
cargo_args = "--release -q"


def get_contributors():
    r = requests.get(url)
    return r.json()
    # contributors = r.json()
    # contri_buf = []
    #
    # for contributor in contributors:
    #     contri_buf.append(contributor["avatar_url"])
    #
    # return contri_buf


def main():
    contributors = get_contributors()
    # print(avatar_url_buf)

    for contributor in contributors:
        avatar_url = contributor["avatar_url"]
        username = contributor["login"]
        print(username)
        for format in formats:
            # Install to tmp.jpeg
            os.system(f"curl -s -o tmp.{format} " + avatar_url)
            if (
                os.system(
                    f"cargo run {cargo_args} -- 'tmp.{format}' {rascii_args} -C {username}"
                    + " >/dev/null 2>&1"
                )
                == 0
            ):
                os.system(
                    f"cargo run {cargo_args} -- 'tmp.{format}' {rascii_args} -C {username}"
                )  # Doing the entire calculation again
                # Insanely inefficient, but It literally has no effect on the user (almost)
                os.system(f"rm tmp.{format}")
                break
            else:
                os.system(f"rm tmp.{format}")
                # print(f"{format} failed")
                # print("trying other formats")
                continue
        print("\n")  # Two newlines


if __name__ == "__main__":
    main()
