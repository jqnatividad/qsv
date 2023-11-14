import hashlib


def md5hash(*args):
    s = ",".join(args)
    return hashlib.md5(s.encode("utf-8")).hexdigest()
