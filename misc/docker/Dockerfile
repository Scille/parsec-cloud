FROM python:3.6.1-alpine

RUN apk add --update --no-cache \
  build-base \
  libffi-dev \
  postgresql-dev \
  fuse-dev

RUN mkdir parsec-cloud
ADD . /parsec-cloud
WORKDIR /parsec-cloud

RUN pip install -r requirements.txt -e .[fuse]

WORKDIR /parsec-cloud/parsec

ENTRYPOINT ["parsec"]
CMD ["backend", "-l", "DEBUG"]
