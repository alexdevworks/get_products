from setuptools import sandbox
from os import path

import pprint
import csv
import unittest

# run setup.py
sandbox.run_setup('setup.py', ['build'])

products = []
i = 0;

output_path = 'screening/data/out.csv'

assert path.exists(output_path) is True
assert path.isfile(output_path) is True

with open(output_path) as read_obj:
    reader = csv.DictReader(read_obj)
    for row in reader:
        product = {}

        product['SKU'] = row['SKU']
        product['designation'] = row['Product Designation']
        product['price'] = int(row['Unit Price (USD)'])
        product['Description'] = "{0} ({1}), {2}".format(product['designation'],product['SKU'],"${:,.0f}".format(product['price']))
        
        products.append(product)
       
# check if everything in the excel file is included.
assert len(products) is 3

# Check if a product has sku, designation, price, and description.
for product in products:
    assert len(product) is 4

pp = pprint.PrettyPrinter(indent=4)
pp.pprint(products)