# Define the python interpreter
PYTHON = python3
# Define pip
PIP = pip3
# This target will install necessary python packages listed in the requirements.txt file
install:
	$(PIP) install -r requirements.txt
# This target will run the tests in the test_main.py file
test:
	$(PYTHON) -m unittest test_main.py
# This target will run the main.py file
run:
	$(PYTHON) main.py
# This target will run both the tests and then if they succeed, run the main.py file
all: test run
.PHONY: install test run all
