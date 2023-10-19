FROM python:3.11-alpine as builder

WORKDIR /build

RUN pip install --user pipenv
ENV PIPENV_VENV_IN_PROJECT=1

ADD Pipfile.lock Pipfile /build/

RUN /root/.local/bin/pipenv install --deploy

FROM python:3.11-alpine

WORKDIR /app

RUN addgroup -g 1000 kronos \
    && adduser -G kronos -u 1000 -s /bin/sh -D -H kronos

COPY --from=builder /build/.venv/lib/ /usr/local/lib

COPY kronos /app/kronos/

USER kronos

CMD ["python", "-u", "-m", "kronos.main"]