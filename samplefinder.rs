use std::io;
use std::fs;
use std::path::PathBuf;
use reqwest::header::{HeaderMap, USER_AGENT};
use scraper::{Html, Selector};

fn main() {
    // Ask the user for the sample name and extension
    println!("Enter the sample name:");
    let mut sample_name = String::new();
    io::stdin()
        .read_line(&mut sample_name)
        .expect("Failed to read line");
    sample_name = sample_name.trim().to_lowercase();

    println!("Enter the sample extension (e.g. wav, mp3, etc.):");
    let mut extension = String::new();
    io::stdin()
        .read_line(&mut extension)
        .expect("Failed to read line");
    extension = extension.trim().to_lowercase();

    // Build the path to the sample directory
    let sample_directory = PathBuf::from("/path/to/sample/directory");

    // Search for samples with the given name and extension
    let mut found_samples = vec![];
    let search_pattern = format!("*{}*.{}", sample_name, extension);
    for entry in fs::read_dir(sample_directory).unwrap() {
        let path = entry.unwrap().path();
        if path.is_file() && path.to_str().unwrap().to_lowercase().ends_with(&search_pattern) {
            found_samples.push(path);
        }
    }

    // Print the paths of the found samples
    if found_samples.is_empty() {
        println!("No samples were found with the name '{}'", sample_name);
    } else {
        println!("The following samples were found with the name '{}':", sample_name);
        for sample_path in found_samples {
            println!("{}", sample_path.display());
        }
    }

    // Ask the user if they want to search for the sample pack online
    println!("Do you want to search for the sample pack online? (y/n)");
    let mut search_online = String::new();
    io::stdin()
        .read_line(&mut search_online)
        .expect("Failed to read line");
    search_online = search_online.trim().to_lowercase();

    if search_online == "y" {
        // Build the URL to search for the sample name on Google
        let query = format!("{} sample pack", sample_name.replace(" ", "+"));
        let google_url = format!("https://www.google.com/search?q={}&num=10", query);

        // Set the user agent header to avoid being blocked by Google
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, "reqwest".parse().unwrap());

        // Send a GET request to Google and parse the response HTML
        let res = reqwest::blocking::Client::new()
            .get(&google_url)
            .headers(headers)
            .send()
            .unwrap();
        let document = Html::parse_document(&res.text().unwrap());

        // Define a selector for the search result links and extract the href attributes
        let link_selector = Selector::parse("a").unwrap();
        let links = document
            .select(&link_selector)
            .map(|link| link.value().attr("href").unwrap_or(""))
            .collect::<Vec<_>>();

        // Search the links for the name of the sample pack and print the result
        let pack_name = links
            .iter()
            .find(|link| link.contains("sample pack"))
            .map(|link| {
                let start_index = link.find("q=")
                    .map(|i| i + 2)
                    .unwrap_or(0)
			);
    let end_index = link.find("&")
        .map(|i| i)
        .unwrap_or(link.len());
    link[start_index..end_index].replace("+", " ")
});

match pack_name {
    Some(name) => println!("The sample '{}' is part of the '{}' sample pack.", sample_name, name),
    None => println!("No sample pack was found for the sample '{}'.", sample_name),
}

// Ask the user if they want to search for the sample on Splice or SampleFocus
println!("Do you want to search for the sample on Splice or SampleFocus? (y/n)");
let mut search_online = String::new();
io::stdin()
    .read_line(&mut search_online)
    .expect("Failed to read line");
search_online = search_online.trim().to_lowercase();

if search_online == "y" {
    // Build the URL to search for the sample on Splice
    let splice_query = format!("https://splice.com/sounds/search?q={}", sample_name.replace(" ", "+"));

    // Send a GET request to Splice and parse the response HTML
    let res_splice = reqwest::blocking::get(&splice_query).unwrap();
    let splice_document = Html::parse_document(&res_splice.text().unwrap());

    // Define a selector for the search result links and extract the href attributes
    let splice_link_selector = Selector::parse("a").unwrap();
    let splice_links = splice_document
        .select(&splice_link_selector)
        .map(|link| link.value().attr("href").unwrap_or(""))
        .collect::<Vec<_>>();

    // Search the links for the name of the sample and print the result
    let splice_sample_link = splice_links
        .iter()
        .find(|link| link.contains("/sounds/"))
        .map(|link| format!("https://splice.com{}", link));

    match splice_sample_link {
        Some(link) => println!("The sample '{}' was found on Splice: {}", sample_name, link),
        None => println!("No sample was found for the sample '{}' on Splice.", sample_name),
    }

    // Build the URL to search for the sample on SampleFocus
    let sample_focus_query = format!("https://samplefocus.com/search?q={}", sample_name.replace(" ", "+"));

    // Send a GET request to SampleFocus and parse the response HTML
    let res_sample_focus = reqwest::blocking::get(&sample_focus_query).unwrap();
    let sample_focus_document = Html::parse_document(&res_sample_focus.text().unwrap());

    // Define a selector for the search result links and extract the href attributes
    let sample_focus_link_selector = Selector::parse("a").unwrap();
    let sample_focus_links = sample_focus_document
        .select(&sample_focus_link_selector)
        .map(|link| link.value().attr("href").unwrap_or(""))
        .collect::<Vec<_>>();

    // Search the links for the name of the sample and print the result
    let sample_focus_sample_link = sample_focus_links
        .iter()
        .find(|link| link.contains("/sample/"))
        .map(|link| format!("https://samplefocus.com{}", link));

    match sample_focus_sample_link {
        Some(link) => println!("The sample '{}' was found on SampleFocus: {}", sample_name, link),
        None => println!("No sample was found for the sample '{}' on SampleFocus.", sample_name),
    }
}
####Functions_ToDO
import requests
from bs4 import BeautifulSoup

# Lista de sitios web donde se puede buscar información de muestra
sites = {
    'Splice': 'https://splice.com/sounds/search?q={}',
    'SampleFocus': 'https://samplefocus.com/search?term={}',
    'Loopmasters': 'https://www.loopmasters.com/search?format[]=wav&category[]=14&search_term={}',
    # Agregar más sitios web aquí si se desea
}

# Función para buscar información de muestra en un sitio web específico
def search_site(site, query):
    url = sites[site].format(query)
    response = requests.get(url)
    soup = BeautifulSoup(response.content, 'html.parser')
    # Aquí se puede agregar código para analizar la página web y obtener información de muestra
    return soup

# Función para buscar información de muestra en múltiples sitios web a la vez
def search_multiple_sites(query):
    results = {}
    for site in sites.keys():
        results[site] = search_site(site, query).prettify()
    return results

# Función para buscar detalles adicionales sobre una muestra
def search_details(query):
    # Aquí se puede agregar código para buscar detalles sobre la muestra en otra API o sitio web
    details = {
        'duration': '5 seconds',
        'quality': '24-bit, 44.1 kHz',
        'genre': 'Electronic'
    }
    return details
#######TODO
# Función para imprimir los resultados de la búsqueda y los detalles de la muestra
def print_results(query):
    # Buscar información de muestra en múltiples sitios web
    results = search_multiple_sites(query)
    for site, content in results.items():
        print(f'Results from {site}:')
        print(content)
    # Buscar detalles adicionales sobre la muestra
    details = search_details(query)
    print('Details:')
    for key, value in details.items():
        print(f'{key.capitalize()}: {value}')

# Función para permitir que el usuario seleccione de qué sitio web buscar información
def select_site():
    print('Select a site to search:')
    for i, site in enumerate(sites.keys()):
        print(f'{i+1}. {site}')
    choice = int(input())
    site = list(sites.keys())[choice-1]
    return site

# Función para buscar información de muestra en un sitio web específico seleccionado por el usuario
def search_selected_site(query):
    site = select_site()
    results = search_site(site, query).prettify()
    print(f'Results from {site}:')
    print(results)

# Ejemplo de uso
query = 'drum samples'
print_results(query)
search_selected_site(query)
