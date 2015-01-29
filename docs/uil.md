# UIL - Design -

**Objectifs:**

 * Externaliser:
    * le layout de nos vues.
    * le style et l'avoir indépendant des vues.
    * les dépendances vers les assets utilisées
 * Offrir de la flexibilité à la fois sur le layout et le style.

**Non-objectifs:**

 * Décrire le game play.
 * Binding entre les modèles et la vue

   *-> Cette partie sera faite programmatiquement en Rust
      par un système de Query sur le markup et en attachant le modèle.
      (cf document par encore écris)*

**Découpage:**

UIL regroupe les différents composants suivants:

 * `libs`
 * `style`
 * `markup`

En terme de structure de fichiers, on pourrais mapper chaque composant
avec un dossier à part.

## Markup (core d'UIL)

Notre markup s'inspire beaucoup d'HTML. On va en garder le language (xml),
la plus part des algorithmes dont celui de layout (à confirmer après avoir vu
les exemples de flo) et la structure (DOM en beaucoup moins générique).

Au lieu d'évoquer les nombreuses différences entre notre markup et HTML,
on va décrire notre language au travers d'exemples.

Commençons avec la vue principal quand un joueur est connecté. On veut offrir
la possibilité au joueur de configurer quelques options avant de jouer et
éventuellement de quitter.

```xml
<view name="main">
    <h1>Slayers Online</h1>
    <group>
        <button name="Play" goto-view="connect" action="play"/>
        <button name="Options" goto-view="options"/>
        <button name="Quit" action="quit"/>
    </group>
</view>
```

Le premier tag que l'on voit défini une `view` nommé `main`. Tout ce qui est contenu
dans ce tag sera rendu directement à l'écran lorsque le joueur arrivera sur cette
`view`. Par défaut notre moteur charge la vue nommé `main`.

Cette vue contient un header `h1` contenant du texte. Jusque là, rien de très spécial.
Elle contient aussi un `group`. Ce groupe permet d'aggréger des éléments mais n'a aucune
sémantique particulière: son rôle est purement visuel et structurel.
Son rôle visuel deviendra plus clair dans la partie sur le **Style**.

Enfin on a plusieurs `button`. Un bouton comme son nom l'indique est un tag cliquable
qui possède plusieurs attributs intéressant:
 * `goto-view` qui permet de demander au `Router` de changer de vue lorsqu'on clique
    sur le bouton.
 * `action` qui permet de demander une action au moteur, ici ce sera quitter le jeu ou
    bien tenter de rejoindre la map sur laquelle son perso était resté lors de
    notre dernière connection. *([liste complète](actions_uil.markdown))*

Maintenant on aimerait enrichir ce menu en y ajoutant la liste des amis. Cependant,
comme on voudrait pouvoir faire la même chose sur plusieurs vue, on va introduire
un nouveau tag: le `template`.

```xml
<view name="main">
    <!-- as before -->
    <template path="amis" />
</view>
```

Ici on indique à notre moteur que l'on va utiliser un template, et le moteur
va tenter d'inclure à cet endroit le template s'appelant `amis`.

Il se pose donc la question de "Comment notre moteur résout-il la recherche du template
 `amis` ?". C'est actuellement très simple. Toutes les fichiers de markup sont chargées
par le moteur avant leur affichage et durant la résolution des conflits tout les templates
sont insérés là où il sont utilisé.

Un `template` se défini de la façon suivante: Il ne doit pas être à l'intérieur
d'un tag `view`. Il peut être dans le même fichier qu'une vue ou pas (cela n'a aucune
importance). En gros cela ressemble à ceci:

```xml
<template name="amis">
    <!-- whatever we want -->
</template>
```

## Style



## Libs

Ce composant permet de définir les dépendances sur les resources et offre un nommage pour y accéder.
Par "resource" on inclus:

 * La définition d'une constante
 * Le chemin d'accès à un fichier

Ces définitions sont dans un namespace particulier définit à la déclaration.

Tous les fichiers
Un fichier avec l'extension `.
```
fonts {
    default: {
        path: "path/to/default-font.png",
        size:

}
```
