from devimports import *


@app.post("/api/licences/get-licences")
@auto_return
def get_licences():
    return {
        "values": []
    }
