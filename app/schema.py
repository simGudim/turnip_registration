from pydantic import BaseModel
from typing import List, Optional

## used to insert into the DB
class User(BaseModel):
    username: str
    first_name: str
    last_name: str
    date_of_birth: str
    password: str
    email: str
    

## used to retrive from the database
class ShowUser(BaseModel):
    username: str
    email: str

    class Config():
        orm_mode = True