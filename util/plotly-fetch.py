#!/usr/bin/env python3

import requests, json, os

folder = "force-graphs"
os.makedirs(folder, exist_ok=True)

def download_plot(url):
    plots = requests.get(url).json()
    filename = plots['filename']
    png = requests.get(plots['image_urls']['default'])
    file = folder + "/" + filename + ".png"
    print("downloading: ", file)
    with open(file, "wb") as f:
        f.write(png.content)

def download_folder_plots(url):
    res = requests.get(url, params = {
        "user": "haata",
        "page": "1",
        "page_size": "10000",
    }).json()
    children = res['children']
    for child in children['results']:
        urls = child['api_urls']
        if 'plots' in urls:
            download_plot(urls['plots'])
        if 'folders' in urls:
            print("entering folder: " + child['filename'])
            download_folder_plots(urls['folders'])
download_folder_plots("https://api.plotly.com/v2/folders/home")
