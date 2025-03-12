from typing import Tuple, Dict

def parse_env_file_contents(contents: str) -> Tuple[Dict[str, str], Dict[str, str]]:
    """
    Parses the contents of a .env file, returning:
    - env_dict: key-value pairs
    - env_comments: key-comment pairs (if any)
    """
    env_dict = {}
    env_comments = {}
    comment_buffer = []

    for line in contents.splitlines():
        stripped = line.strip()
        if not stripped:
            comment_buffer = []
            continue

        if stripped.startswith("#"):
            comment_buffer.append(stripped.lstrip("# "))
            continue

        if "=" in stripped:
            key, value = stripped.split("=", 1)
            key = key.strip()
            value = value.strip()
            env_dict[key] = value
            if comment_buffer:
                env_comments[key] = "\n".join(comment_buffer)
            comment_buffer = []

    return env_dict, env_comments
