from devimports import *


@app.post("/api/users/get-users")
@auto_return
def get_users():
    return {
        "values": []
    }
