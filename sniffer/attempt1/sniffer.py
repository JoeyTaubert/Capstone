"""
Author: Joey Taubert

All code is my own work, except in areas where credit is clearly defined.
"""

# Reference: https://medium.com/@vworri/extracting-the-payload-from-a-pcap-file-using-python-d938d7622d71
# Her GitHub for reference: https://github.com/Vworri########################################


now = datetime.now()
print("Capturing started at",now)

os.system("tshark  -i 5 -T fields -e  data.data -e frame.time -w Eavesdrop_Data.pcap -F pcap -c 1000")
#os.system("tshark -F {output file format} -r {input file} -w {output file}")
# tshark -F k12text -r -w