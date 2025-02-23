import os
import glob
import pandas as pd
import matplotlib.pyplot as plt

def generate_png_from_csv(csv_file):
    """
    Funzione che legge il file CSV, genera un grafico e salva il grafico come file PNG.
    """
    
    try:
        # Legge i dati dal file CSV
        data = pd.read_csv(csv_file)
        
        # Conversione del campo 'timestamp' in formato datetime (se necessario)
        data['timestamp'] = pd.to_datetime(data['timestamp'])

        # Esempio: se il CSV contiene colonne 'x' e 'y', crea un grafico a linee
        plt.figure(figsize=(12, 6))
        
        # plt.plot(data['timestamp'], data['response_time'], marker='o')
        plt.scatter(data['timestamp'], data['response_time'], alpha=0.6, edgecolor='k')

        plt.title(f'Grafico generato da {os.path.basename(csv_file)}')
        plt.xlabel('Timestamp')
        plt.ylabel('Tempo di risposta (ms)')
        plt.grid(True)

        # Costruisce il nome del file PNG
        png_file = os.path.splitext(csv_file)[0] + '.png'
        plt.savefig(png_file)
        plt.close()
        
        print(f'File PNG generato: {png_file}')
    except Exception as e:
        print(f'Errore nella generazione del PNG per {csv_file}: {e}')

def main(directory):
    """
    Cerca tutti i file CSV nella directory specificata e, per ognuno,
    se non esiste già un file PNG con lo stesso nome, richiama la funzione
    per generarlo.
    """

    # Trova tutti i file con estensione .csv nella directory (non ricorsivamente)
    csv_files = glob.glob(os.path.join(directory, '*.csv'))
    
    if not csv_files:
        print("Nessun file CSV trovato nella directory.")
    
    for csv_file in csv_files:
        png_file = os.path.splitext(csv_file)[0] + '.png'

        if not os.path.exists(png_file):
            print(f"Generazione del file PNG per: {csv_file}")
            generate_png_from_csv(csv_file)
        else:
            print(f"Il file {png_file} esiste già.")

if __name__ == '__main__':
    main('./data')
