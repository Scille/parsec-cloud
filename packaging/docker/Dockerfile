FROM python:3.7-slim-buster

RUN pip install --no-cache-dir parsec-cloud[backend]
EXPOSE 6777
ENTRYPOINT ["parsec", "backend"]
CMD ["run"]

# ADD . /
# RUN pip install -e .[backend]
# RUN python setup.py
# EXPOSE 6777
# CMD ["python", "parsec backend run"]
