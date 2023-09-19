"""
Author: Joey Taubert

All code is my own work, except in areas where credit is clearly defined.
"""

import subprocess
import os
from datetime import datetime

# os.system("tshark  -i 5 -T fields -e  data.data -e frame.time -w Eavesdrop_Data.pcap -F pcap -c 1000")
# os.system("tshark -F {output file format} -r {input file} -w {output file}")
# tshark -F k12text -r -w

print("Interface List")
print("--------------")
os.system("tshark -D")
interface = input("Which number interface would you like to use?")

now = datetime.now()
now = now.strftime("%d-%m-%Y%H-%M-%S")

# Windows equivalent to pwd, grab the  (credit to ChatGPT for this line)
current_directory = subprocess.check_output("cd", shell=True, universal_newlines=True).strip()
filename = "tshark" + now
output_file = "\"" + current_directory + "\\" + filename + ".pcap" + "\""
print("Outputting at:", output_file)

capture = "tshark -i " + str(interface) + " -c 1000 -w " + output_file + " -F libpcap"

print("Full command:", capture)
os.system(capture)
