FROM python:3.11-alpine as builder

WORKDIR /build

RUN pip install --user pipenv
ENV PIPENV_VENV_IN_PROJECT=1

ADD Pipfile.lock Pipfile /build/

RUN /root/.local/bin/pipenv install --deploy

FROM python:3.11-alpine

RUN addgroup -g 1000 olympus \
    && adduser -G olympus -u 1000 -s /bin/sh -D -H olympus

COPY --from=builder /build/.venv/lib/ /usr/local/lib

COPY bot /app/bot/

USER olympus

CMD ["python", "-u", "/app/bot/main.py"]