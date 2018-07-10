import logging
import os
import sys
import threading

import boto3
from pyupdater.uploader import BaseUploader
from pyupdater.utils.exceptions import UploaderError

log = logging.getLogger(__name__)


class S3Uploader(BaseUploader):

    name = "S3"
    author = "Scille"

    def init_config(self, config):
        for config_key in ["access_key", "secret_key", "region_name", "bucket_name"]:
            config_value = config.get(config_key)
            if not config_value:
                config_value = os.environ.get("PYU_AWS_" + config_key.upper())
            if config_value:
                setattr(self, config_key, config_value)
            else:
                raise UploaderError(
                    config_key.replace("_", " ").capitalize() + " is not set", expected=True
                )
        self._connect()

    def set_config(self, config):
        for config_key in ["access_key", "secret_key", "region_name", "bucket_name"]:
            config_value = config.get(config_key)
            config_value = self.get_answer(
                "Please enter a " + config_key.replace("_", " "), default=config_value
            )
            config[config_key] = config_value

    def _connect(self):
        self.s3 = boto3.client(
            "s3",
            region_name=self.region_name,
            aws_access_key_id=self.access_key,
            aws_secret_access_key=self.secret_key,
        )
        self.s3.head_bucket(Bucket=self.bucket_name)

    def upload_file(self, filename):
        try:
            self.s3.upload_file(
                filename,
                self.bucket_name,
                filename,
                ExtraArgs={"ACL": "public-read"},
                Callback=ProgressPercentage(filename),
            )
            log.debug("Uploaded {}".format(filename))
            return True
        except Exception as err:
            log.error("Failed to upload file")
            log.debug(err, exc_info=True)
            self._connect()
            return False


class ProgressPercentage(object):
    def __init__(self, filename):
        self._filename = filename
        self._size = float(os.path.getsize(filename))
        self._seen_so_far = 0
        self._lock = threading.Lock()

    def __call__(self, bytes_amount):
        with self._lock:
            self._seen_so_far += bytes_amount
            percentage = (self._seen_so_far / self._size) * 100
            sys.stdout.write("\r%s / %s  (%.2f%%)" % (self._seen_so_far, self._size, percentage))
            sys.stdout.flush()
