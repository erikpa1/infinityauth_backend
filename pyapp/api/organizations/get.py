from devimports import *


@app.post("/api/organizations/get-organizations")
@auto_return
def get_organizations():
    return {
        "values": []
    }
