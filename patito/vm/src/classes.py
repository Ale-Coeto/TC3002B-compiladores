from dataclasses import dataclass
from typing import List, Union

from src.quad_codes import QuadCode

@dataclass
class Quad:
    operator: QuadCode
    left_operand: Union[int, str]
    right_operand: Union[int, str]
    result: Union[int, str]

@dataclass
class Function:
    name: str
    start_index: int
    local_count: int
    temp_count: int

@dataclass
class Constant:
    address: int
    type: str
    value: Union[int, float, str]

@dataclass
class Program:
    quads: List[Quad]
    functions: List[Function]
    constants: List[Constant]