import setuptools


def get_install_reqs():
    with open('requirements.txt', 'r') as f:
        install_reqs = f.read().splitlines()
    return install_reqs


install_reqs = get_install_reqs()

setuptools.setup(
    name='ficus',
    version='1.0.0',
    author='Aero',
    author_email='aerooneq@yandex.ru',
    description='Some Process Mining techniques implementations',
    long_description='Some Process Mining techniques implementations',
    long_description_content_type="text/markdown",
    license='private',
    packages=['ficus',
              'ficus.legacy',
              'ficus.legacy.discovery',
              'ficus.legacy.log',
              'ficus.legacy.analysis',
              'ficus.legacy.analysis.patterns',
              'ficus.legacy.analysis.common',
              'ficus.legacy.pipelines',
              'ficus.legacy.pipelines.analysis',
              'ficus.legacy.pipelines.analysis.patterns',
              'ficus.legacy.pipelines.serialization',
              'ficus.legacy.pipelines.discovery',
              'ficus.legacy.pipelines.filtering',
              'ficus.legacy.pipelines.mutations',
              'ficus.legacy.pipelines.contexts',
              'ficus.legacy.pipelines.start',
              'ficus.legacy.mutations',
              'ficus.legacy.filtering',
              'ficus.grpc_pipelines',
              'ficus.grpc_pipelines.models'],
    install_requires=install_reqs,
)
