current_version = ""
current_tag = ""
with open("../VERSION", 'r') as f:
    current_version = f.readlines()[0].replace('\n', '');

current_tag = current_version.split('-')[-1]

version = str(int(current_version.split('-')[0].replace('.', '')) + 1)
string = ''
for i in version:
    string = string + version[0] + '.'

if string[:-1] == '.':
    string[-1] = ''

with open("../VERSION", 'w') as file:
    file.write('.'.join(k for k in version) + '-' + current_tag)