FROM python:3.7-slim-buster

ADD . .
RUN pip install --no-cache-dir -e .[server]
EXPOSE 6777
ENTRYPOINT ["parsec", "server"]
CMD ["run"]
