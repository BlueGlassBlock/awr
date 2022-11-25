from .awr import GroupMessage
from loguru import logger


def callback(e):
    logger.debug(repr(e))
