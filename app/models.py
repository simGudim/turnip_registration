from .utils.database import Base
from sqlalchemy import Column, Integer, String, ARRAY
from sqlalchemy.sql.schema import ForeignKey
from sqlalchemy.orm import relationship

class User(Base):
    __tablename__ = "users"
    id = Column(Integer, primary_key = True, index = True)
    username = Column(String)
    first_name = Column(String)
    last_name = Column(String)
    date_of_birth = Column(String)
    password = Column(String)
    email = Column(String)
    description = Column(String)
    followers_id = Column(ARRAY(Integer))
