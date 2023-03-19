import logging
import traceback

from functools import wraps


def auto_return(f):
    @wraps(f)
    def innerRegistration(*args, **kwargs):

        try:
            tmpValue = f(*args, **kwargs)
            if isinstance(tmpValue, tuple):
                return {"status": tmpValue[0]}, tmpValue[1]
            elif isinstance(tmpValue, dict):
                return tmpValue, 200
            else:
                return {
                           "status": tmpValue
                       }, 200


        except Exception as e:
            traceback.print_exc()
            return {
                       "why": str(e)
                   }, 500

    return innerRegistration
