class SerdeError(Exception):
    pass


class SerdeValidationError(SerdeError):
    def __init__(self, errors: dict):
        self.errors = errors


class SerdePackingError(SerdeError):
    pass
