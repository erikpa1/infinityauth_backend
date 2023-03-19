from devimports import *


@app.post("/api/users/try-login")
@auto_return
def try_login():
    user_name = request.form.get("user_name", "")
    pass_hash = request.form.get("password", "")

    return {
        "is_valid": False,
        "user_role": "admin",
        "user_data": {
            "user_name": "Erik",
            "user_surname": "Palencik",
        }
    }
