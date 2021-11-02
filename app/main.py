from fastapi import FastAPI
from . import models
from .utils.database import engine
from .utils.hashing import Hash
from .routers import users

app = FastAPI()
models.Base.metadata.create_all(engine)
hashing = Hash()

app.include_router(users.router)

if __name__ == "__main__":
    app.run()