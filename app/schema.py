from pydantic import BaseModel
from typing import List, Optional

class User(BaseModel):
    username: str
    password: str
    email: str
    description: str


class ShowUser(BaseModel):
    username: str
    email: str

    class Config():
        orm_mode = True