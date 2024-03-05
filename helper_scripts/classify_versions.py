import re

versions = []

with open("../Versions", 'r') as versions_file:
    for line in versions_file.readlines():
        versions += [ ver.strip() for ver  in line.split(",") ]

versions = list(set(versions))
    
version_regexs = [\
    # data based
    # re.compile(r'^20(\d{2})-(\w{3})-(\d{2})-([\w\d]+)$'), \
    # re.compile(r'^([\w\d-]+)?20(\d{2})[\.-](\d{1,2})[\.-](\d{1,2})([\.\w\d-]+)?$'), \
    # version based
    # re.compile(r'^v?(\d+)\.(\d+)(\.(\d+))?(\.(\d+))?$'), \
    # re.compile(r'^v?(\d+)\.(\d+)(\.(\d+))?-?(\w+)?(\d+)?$'), \
    # re.compile(r'^(\d{2})-(\d{2})-20(\d{2}).(\d+)$') \
    ]

year_regex = re.compile(r'(\D|^)20\d{2}(\D|$)')

year_based_versions = []
version_based = []

for ver in versions:
    if year_regex.match(ver):
        year_based_versions.append(ver)
    else:
        if '202' in ver:
            print(f"why? {ver}")
        version_based.append(ver)


def printContidion(version: str):
    return True