import os
from flask import Flask
from flask.templating import render_template
import datetime

app = Flask(__name__)
path = os.path.dirname(os.path.realpath(__file__))

global latest_date
global trigger;

@app.route("/")
def index():
    return render_template("receive.html")

@app.route("/status", methods=["GET","POST"])
def status():
    if trigger:
        return "true"
    else:
        return "false"

@app.route("/getdate", methods=["GET","POST"])
def getdate():
    global latest_date
    if latest_date is not None:
        return latest_date
    else:
        return "---"

@app.route("/trigger")
def trigger():
    global trigger
    global latest_date
    trigger = True
    latest_date = datetime.datetime.now().strftime("%d/%m/%Y %H:%M:%S")
    return ""

@app.route("/untrigger")
def untrigger():
    global trigger
    trigger = False
    return ""

@app.route("/receive")
def receive(): 
    return render_template("receive.html")
   
if __name__ == '__main__':
    trigger = False
    latest_date = None
    app.run(host = "172.20.10.4", debug=True)