import os
from typing import Annotated

from fastapi import Header, HTTPException, status

API_KEY = os.getenv("API_KEY")

if not API_KEY:
    raise ValueError(
        "API_KEY environment variable is required. "
        "Please set API_KEY in your environment variables."
    )


async def verify_api_key(x_api_key: Annotated[str, Header()]) -> str:
    """
    Dependency para verificar el header x-api-key

    Args:
        x_api_key: El valor del header x-api-key

    Returns:
        str: La API key si es válida

    Raises:
        HTTPException: Si la API key no es válida o está ausente
    """
    if not x_api_key:
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail="API key is required",
            headers={"WWW-Authenticate": "ApiKey"},
        )

    if x_api_key != API_KEY:
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail="Invalid API key",
            headers={"WWW-Authenticate": "ApiKey"},
        )

    return x_api_key
