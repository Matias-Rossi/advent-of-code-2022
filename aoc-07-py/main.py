from enum import Enum

class Type(Enum):
    FILE = 1
    DIR = 2

class File:
    def __init__(self, name, size):
        self.name = name
        self.size = size
        self.type = Type.FILE

    def get_size(self):
        return self.size

class Dir:
    def __init__(self, name):
        self.name = name
        self.contents = {}
        self.type = Type.DIR

    def get_size(self):
        size = 0
        for k, v in self.contents.items():
            size += v.get_size()
        return size

    def get_or_create_subdir(self, subdir_name, directories):
        if subdir_name in self.contents:
            return self.contents[subdir_name]
        else:
            new_dir = Dir(subdir_name)
            self.contents[subdir_name] = new_dir
            directories.append(new_dir)
            return new_dir

    def get_or_create_file(self, file_name, file_size):
        if file_name in self.contents:
            return self.contents[file_name]
        else:
            new_file = File(file_name, file_size)
            self.contents[file_name] = new_file
            return new_file

def handle_user_input(tokens, current_path, directories):
    match tokens[1]:
        case 'cd':
            cd_target = tokens[2]
            match cd_target:
                case '/':
                    go_to_root_dir(current_path)
                case '..':
                    go_up_one_dir(current_path)
                case other:
                    go_to_subdir(current_path, cd_target, directories)
    
def go_to_root_dir(current_path):
    current_path = current_path[:1]

def go_up_one_dir(current_path):
    del current_path[-1]

def go_to_subdir(current_path, subdir_name, directories):
    new_subdir = current_path[-1].get_or_create_subdir(subdir_name, directories)
    current_path.append(new_subdir)

def read_ls(tokens, current_path, directories):
    current_dir = current_path[-1]
    match tokens[0]:
        case 'dir':
            subdir_name = tokens[1]
            current_dir.get_or_create_subdir(subdir_name, directories)
        case other:
            # file
            file_name = tokens[1]
            file_size = int(tokens[0])
            current_dir.get_or_create_file(file_name, file_size)


if __name__ == "__main__":
    with open('resources/input.txt', 'r') as file:
        root_dir = Dir("/")
        current_path = [root_dir]
        directories = []

        while True:
            line = file.readline().strip()
            if not line: break

            # Get input
            tokens = line.split(' ')
            match tokens[0]:
                case '$':
                    handle_user_input(tokens, current_path, directories)
                case other:
                    read_ls(tokens, current_path, directories)
            

        # Part A
        size_of_dirs_leq_100000 = 0

        for dir in directories:
            dir_size = dir.get_size()
            if dir_size <= 100000:
                size_of_dirs_leq_100000 += dir_size

        print(f"Sum of total sizes of directories with size of at most 100000: {size_of_dirs_leq_100000}") # 1449447

        # Part B
        total_space = 70000000
        needed_space = 30000000
        used_space = root_dir.get_size()
        space_to_be_freed = used_space - ( total_space - needed_space )

        # find smallest dir that would free up enough size
        smallest_dir_name, smallest_dir_size = ("", 70000000)

        for dir in directories:
            dir_size = dir.get_size()
            if dir_size > space_to_be_freed and dir_size < smallest_dir_size:
                smallest_dir_name = dir.name;
                smallest_dir_size = dir_size;

        print(f'Size of dir to be deleted: {smallest_dir_size}')




        
        
        