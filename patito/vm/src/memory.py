
class LocalMemory:
    def __init__(self):
        self.memory = {}
        self.int_params = 0
        self.float_params = 0
    
    def get_value(self, address):
        return self.memory.get(address)
    
    def set_value(self, address, value):
        self.memory[address] = value

class Memory:
    def __init__(self, constants):
        self.constants = {const.address: const.value for const in constants}
        self.global_memory = {}
        self.local_memory = []
        self.local_memory.append(LocalMemory())

        self.constants_range = (6000, 7999)
        self.global_range = (1000, 1999)
        self.local_range = (2000, 5999)

        self.global_int_range = (1000, 1499)
        self.global_float_range = (1500, 1999)
        self.local_int_range = (2000, 2499)
        self.local_float_range = (2500, 2999)
        self.temp_int_range = (3000, 3999)
        self.temp_float_range = (4000, 4999)

        self.temp_local_memory = None

    def new_local_memory(self):
        self.temp_local_memory = LocalMemory()

    def use_new_local_memory(self):
        self.local_memory.append(self.temp_local_memory)
        self.temp_local_memory = None

    def release_local_memory(self):
        self.local_memory.pop()
    
    def get_value(self, address):
        if self.constants_range[0] <= address <= self.constants_range[1]:
            return self.constants.get(address)
        elif self.global_range[0] <= address <= self.global_range[1]:
            return self.global_memory.get(address)
        elif self.local_range[0] <= address <= self.local_range[1]:
            return self.local_memory[-1].get_value(address)
        else:
            raise Exception(f"GET - Invalid memory address: {address}")
        
    def set_value(self, address, value):
        if self.global_range[0] <= address <= self.global_range[1]:
            self.global_memory[address] = value
        elif self.local_range[0] <= address <= self.local_range[1]:
            self.local_memory[-1].set_value(address, value)
        else:
            raise Exception(f"SET - Invalid memory address: {address}")

    def is_int(self, address, value):
        if self.global_range[0] <= address <= self.global_range[1]:
            return self.global_int_range[0] <= address <= self.global_int_range[1]
        elif self.local_int_range[0] <= address <= self.local_int_range[1]:
            return True
        elif self.local_float_range[0] <= address <= self.local_float_range[1]:
            return False
        elif self.temp_int_range[0] <= address <= self.temp_int_range[1]:
            return True
        elif self.temp_float_range[0] <= address <= self.temp_float_range[1]:
            return False
        else:
            return isinstance(value, int)
    
    def set_param(self, address):
        value = self.get_value(address)
        if self.is_int(address, value):
            param_address = self.local_int_range[0] + self.temp_local_memory.int_params
            self.temp_local_memory.int_params += 1
        else:
            param_address = self.local_float_range[0] + self.temp_local_memory.float_params
            self.temp_local_memory.float_params += 1

        self.temp_local_memory.set_value(param_address, value)