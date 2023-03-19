from fapp import app

from flask import request


@app.post("/api/users/try-login")
def try_login():
    user_name = request.form.get("user_name", "")
    pass_hash = request.form.get("password", "")

    pass
