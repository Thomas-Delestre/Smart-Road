Option Simple Algo: {

    _ Un vecteur Centre indique quelle voiture se trouve dedans respectant l'ordre d'arrivée. Sous forme de file.
    _ Les voitures sortie du centre vont à vitesse max car plus aucune collision possible, et sont kick du vecteur.
    _ Les voitures tournant à droite fonce, et ne sont jamais prise en compte dans l'algo.
    _ Un booléen indiquera si une voiture a atteint sa voie d'arrivée. Par défaut les voiture tournant a droite seront fixé sur TRUE. 

    Voiture sur voie de départ -> {
        Les voitures ne ralentisse que si : Centre != vide && si au moins une voitures dans le centre ne viennent pas de la même voie qu'elle.
    }

    Voiture au centre -> {
        _ Si la voiture est en première position dans le vect Centre, elle a la priorité et accélère.
        _ En remontant le vect Centre, tant que les voitures viennent de la même voie de départ que la première, elle partage sa priorité et accélère.
        _ Une voiture sortant du centre pour aller sur sa voie d'arrivée accélèrent et est kick du vect Centre, actualisant la position des autres voitures dans celui-ci. 
    }
}



    
