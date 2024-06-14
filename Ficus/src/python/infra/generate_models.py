import os.path
import shutil
import sys
from os import listdir
from os.path import isfile, join


def generate_models():
    package_name = sys.argv[1]
    models_dir = os.path.join(sys.argv[2], 'models')
    protos_dir = sys.argv[3]

    if os.path.exists(models_dir):
        shutil.rmtree(models_dir)

    os.mkdir(models_dir)

    python = 'python3'
    os.popen(f'{python} -m pip install grpcio-tools').read()
    packages = []

    for package in packages:
        os.popen(
            f'{python} -m grpc_tools.protoc -I {protos_dir} --python_out={models_dir}'
            f' --grpc_python_out={models_dir} --pyi_out={models_dir} {protos_dir}/{package}/*.proto').read()

    os.popen(
        f'{python} -m grpc_tools.protoc -I {protos_dir} --python_out={models_dir}'
        f' --grpc_python_out={models_dir} --pyi_out={models_dir} {protos_dir}/*.proto').read()

    common_proto_files = [f for f in listdir(protos_dir) if isfile(join(protos_dir, f))]
    common_files = []
    for file in common_proto_files:
        file_name = os.path.splitext(file)[0]
        common_files.append(f'{file_name}_pb2')
        common_files.append(f'{file_name}_pb2_grpc')

    for common_file in common_files:
        patch_common_types(join(models_dir, f'{common_file}.py'), common_files, package_name)

    for package in packages:
        dir = join(models_dir, package)
        for file in [join(dir, f) for f in listdir(dir) if isfile(join(dir, f))]:
            process_file(file, packages, common_files, package_name)

    init_path = os.path.join(models_dir, '__init__.py')
    if not os.path.exists(init_path):
         with open(init_path, 'w'):
             pass


def patch_common_types(file, common_files, package_name):
    result_lines = []
    with open(file) as fs:
        for line in fs.readlines():
            if "import " in line:
                words = line.split(' ')
                if words[1] in common_files:
                    words[1] = f'{package_name}.models.' + words[1]
                result_lines.append(' '.join(words))
            else:
                result_lines.append(line)

    os.remove(file)
    with open(file, 'w') as fs:
        fs.write('\n'.join(result_lines))


def process_file(file, packages, common_files, package_name):
    result_lines = []
    with open(file) as fs:
        for line in fs.readlines():
            if "from " in line:
                words = line.split(' ')
                if words[1] in packages:
                    words[1] = '..' + words[1]
                result_lines.append(' '.join(words))
            elif "import " in line:
                words = line.split(' ')
                if words[1] in common_files:
                    words[1] = f'{package_name}.models.' + words[1]
                result_lines.append(' '.join(words))
            else:
                result_lines.append(line)

    os.remove(file)
    with open(file, 'w') as fs:
        fs.write('\n'.join(result_lines))


generate_models()
