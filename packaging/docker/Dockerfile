FROM python:3.7-slim-buster

ADD . .
RUN pip install --no-cache-dir -e .[backend]
EXPOSE 6777
ENTRYPOINT ["parsec", "backend"]
CMD ["run"]
