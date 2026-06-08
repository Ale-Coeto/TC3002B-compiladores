
import json
import sys

from src.virtual_machine import VirtualMachine
from src.quad_codes import QuadCode
from src.classes import Constant, Function, Program, Quad

def main():
    json_file = sys.argv[1]

    with open(json_file, "r") as f:
        data = json.load(f)

    program = Program(
        quads=[
            Quad(QuadCode(op), left, right, result)
            for op, left, right, result in data["quads"]
        ],
        functions=[
            Function(**func)
            for func in data["functions"]
        ],
        constants=[
            Constant(**const)
            for const in data["constants"]
        ]
    )

    vm = VirtualMachine(program, verbose=False)
    vm.run()

if __name__ == "__main__":
    main()