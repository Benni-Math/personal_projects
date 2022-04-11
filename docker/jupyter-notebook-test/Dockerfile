FROM python:3.8-slim-buster

WORKDIR /notebooks

COPY requirements.txt requirements.txt

RUN pip3 install -r requirements.txt


# ENV PORT=8888

# EXPOSE 8888

CMD ["jupyter", "notebook","--ip", "0.0.0.0", "--no-browser", "--allow-root", "--NotebookApp.token=abcd"]