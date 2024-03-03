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

sorted_versions = {}

for ver in versions:
    for regex in version_regexs:    
        match = regex.match(ver)
        if(match):
            if regex.pattern not in sorted_versions.keys():
                sorted_versions[regex.pattern] = []
            sorted_versions[regex.pattern].append(ver)
            break
            # print(f"{match.group(1)}:{match.group(2)}:{match.group(2)}")

total_versions = len(versions)

for pattern in sorted_versions.keys():
    count_for_pattern = len(sorted_versions[pattern])
    eg_string = sorted_versions[pattern][0] if count_for_pattern > 0 else ""
    print(f"{eg_string} --- {pattern}: {count_for_pattern}")
    for already_sorted in sorted_versions[pattern]:
        versions.remove(already_sorted)

def printContidion(version: str):
    return True

for vers in versions:
    if printContidion(vers):
        print(vers)

print(f"{total_versions}/{len(versions)}")