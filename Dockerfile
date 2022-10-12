FROM python:latest

#ENV FLASK_APP hello.py

RUN adduser --disabled-password ptodd
USER ptodd

WORKDIR /home/ptodd

#COPY environment.yml environment.yml

#COPY hello.py hello.py

#EXPOSE 5000

