**Cinema**   
    2.1. CRUD film: id, titlu, an apariție, preț bilet, în program. Prețul să fie strict pozitiv.  
    2.2. CRUD card client: id, nume, prenume, CNP, data nașterii (`dd.mm.yyyy`), data înregistrării (`dd.mm.yyyy`), puncte acumulate. CNP-ul trebuie să fie unic.  
    2.3. CRUD rezervare: id, id_film, id_card_client (poate fi nul), data și ora. Clientul acumulează pe card `10%` (parte întreagă) din prețul filmului. Se tipărește numărul total de puncte de pe card. Rezervarea se poate face doar dacă filmul este încă în program.  
    2.4. Căutare filme și clienți. Căutare full text.  
    2.5. Afișarea tuturor rezervărilor dintr-un interval de ore dat, indiferent de zi.  
    2.6. Afișarea filmelor ordonate descrescător după numărul de rezervări.  
    2.7. Afișarea cardurilor client ordonate descrescător după numărul de puncte de pe card.  
    2.8. Ștergerea tuturor rezervărilor dintr-un anumit interval de zile.  
    2.9. Incrementarea cu o valoare dată a punctelor de pe toate cardurile a căror zi de naștere se află într-un interval dat.   