FROM python:3.9.14-bullseye

ENV FLASK_APP hello.py

RUN adduser -D ptodd
USER ptodd

WORKDIR /home/ptodd

COPY environment.yml

COPY hello,py boot.sh ./

EXPOSE 5000

