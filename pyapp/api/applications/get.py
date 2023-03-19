from devimports import *


@app.post("/api/applications/get-applications")
@auto_return
def get_applications():
    return {
        "values": []
    }
