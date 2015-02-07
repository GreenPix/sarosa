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
 * `data-bindings`

En terme de dossiers et de fichiers, on pourrait mapper chaque composant
avec un dossier à part.

Notons que `libs` fournit des définitions statique uniquement pour le
composant `style`.

De la même façon `data-bindings` fournit des défintions dynamique uniquement
pour le composant `markup`.

En résumé:

```
 (static)           libs -> style
(dynamic)  data-bindings -> markup
```




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
        <button goto-view="connect" action="play">Play</buttton>
        <button goto-view="options">Options</buttton>
        <button action="quit">Quit</buttton>
    </group>
</view>
```

Le premier tag que l'on voit défini ici est une `view` nommé `main`. Tout ce qui est
contenu dans ce tag sera rendu directement à l'écran lorsque le joueur arrivera sur
cette `view`. Par défaut le **Routeur** charge la vue nommé `main`.

Cette vue contient un header `h1` contenant du texte. Jusque là, rien de très spécial.
Elle contient aussi un `group`. Ce tag permet d'aggréger des éléments mais n'a aucune
sémantique particulière: son rôle est purement visuel et structurel.
Son rôle visuel deviendra plus clair dans la partie sur le **Style**.

Enfin on a plusieurs `button`. Un bouton comme son nom l'indique est un tag cliquable
qui possède plusieurs attributs intéressant:
 * `goto-view` qui permet de demander au `Router` de changer de vue lorsqu'on clique
    sur le bouton.
 * `action` qui permet de demander une action au moteur, ici ce sera quitter le jeu ou
    bien tenter de rejoindre la map sur laquelle son perso était resté lors de
    notre dernière connection. *([liste complète](actions_uil.md))*

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
par le moteur avant leur affichage et durant la résolution des conflits tout les
templates sont insérés là où il sont utilisé.

Un `template` se défini de la façon suivante:
 * Il ne doit pas être à l'intérieur d'un tag `view`.
 * Il peut être dans le même fichier qu'une vue ou pas
   (cela n'a aucune importance).

En gros cela ressemble à ceci:

```xml
<template name="amis">
    <!-- whatever we want -->
</template>
```

### Tag list

#### line-input

*Exemple:*

```xml
<line-input value="{{tchat.msg}}" key="{{options.keyboard.submitmsg}}"/>
```

*Contexte:* None

*Attributs:*
 - `value` représente le contenu editable de la barre.
 - `key` représente le keyCode de la touche  sur laquelle il faut appuyer
   pour générer un UserEvent avec ce tag. Par défaut, c'est la touche ENTER.

#### progress-bar

*Exemple:*

```xml
<progress-bar value="{{player.xp_pct}}"/>
```

*Contexte:* None

*Attributs:*
 - `value` représente la valeur actuelle de la progress-bar

#### group

*Exemple:*

```xml
<group></group>
```

*Contexte:* None

*Attributs:* None

#### button

*Exemple:*

```xml
<button goto-view="foo" action="bar" key="A"/>
```

*Contexte:* None

*Attributs:*
 - `goto-view` contient le nom d'une vue vers laquelle on va après avoir appuyé sur `key`.
 - `action` contient le nom d'une action parmi celle [disponible](action_uil.md).
 - `key` contient la touche pour trigger le UserEvent. Par défaut, c'est la touche ENTER.

#### template

*Exemple:*

```xml
<template path="toto" class="abc"/>
```

*Contexte:* None

*Attributs:*
 - `path` contient le nom d'un template qui sera inclus à cet endroit précis.
   On ne peut pas utiliser de data-binding ici. (Ca n'a pas de sens).

#### template

*Exemple:*

```xml
<template name="toto">
      <!-- any tag from below -->
</template>
```

*Contexte:* None

*Attributs:*
 - `name` contient et définit le nom du template. (pas de data-binding possible ici non plus)

#### repeat

*Exemple:*

```xml
<repeat iter="{{inventory.items}}" template-name="item"/>
```

*Contexte:* None

*Attributs:*
 - `iter` doit nécessairement être un data-binding. Internement,
   cela correspondra à un trait `IterDataBinder`.
 - `template-name` est le nom du template qui va être utilisé pour chaque
   élements généré par l'iterateur précédent. De plus tout les data-bindings
   défini dans template-name iront chercher d'abord au niveau de l'itérateur
   avant de remonter la scope. Donc un data-binding du type `{{name}}` sera
   équivalent sémantiquement à quelque chose comme `{{inventory.items[i].name}}`.
   Notons que `{{<iter>[i]}}` est illégal.




## Data bindings

Attacher une donnée connu au run-time à un élément du markup se fait via les *data
bindings*. On utilise une grammaire différente pour représenter un tel binding.

Pour l'instant, on supporte uniquement un binding avec une chaine de caractère
dans un namespace particulier. A l'exception du tag `repeat` qui accepte un
iterateur sur des données enfants.

Prenons un premier exemple: si l'on veut ajouter un message personnel pour le joueur
une fois qu'il est connecté on va introduire un `data bindings` de la façon suivante:

```xml
<view name="main">
 - Slayers Online -
 Salut {{player.name}}! Bienvenue sur {{server.name}}.
</view>
```

Pour afficher la liste des amis on ferait comme ceci:

```xml
<view name="amis">
    <repeat iter="{{player.friends}}" template-name="ami">
</view>
<template name="ami">
    <group>
        {{name}} est {{status}}
    </group>
</template>
```

Ici `name` et `status` vont tenter d'accéder au champ d'un élément du tableau
`player.friends` en priorité et si le champ n'est pas disponible à ce namespace
il vont essayer un namespace plus haut, etc.. jusqu'à arriver au namespace global.

Le namespace est séparé par des `.` et le dernier nom représente la variable
qui va être lu et qui va remplacé le texte `{{player.name}}` par le nom du
joueur connecté.

Les data-bindings sont autorisé sur:
* l'attribut `path` d'un `template`
* à l'intérieur d'un texte.
* les attributs `key` et `value` d'un `line-input`
* les attributs `action` et `key` d'un `button`
* l'attribut `value` d'une `progress-bar`

Ils sont interdit (n'auront aucun effet) dans:
* l'attribut `class`
* l'atttribut `goto-view` d'un `button`




## Style

Tous les tags, y compris `view` et `template` supportent un attribut appelé `class`.
Comme on pourrait s'y attendre (et de la même façon qu'en HTML), cet attribut
contient une liste de string séparé par des espaces (ex: `class="foo bar xo"`)
et détermine comment cet élement doit être rendu.

Le style est défini comme un ensemble de règles, chaque règle est composé d'un sélecteur
et d'un corps. Le corps de la règle contient un ensemble de déclaration contenant un mot
clé du language de style avec une valeur.

Un sélecteur est un identifiant de la forme: `.[0-9a-zA-Z\-]+`

Notons que la ressemblance avec le css est faible: nos sélecteurs sont très simplifié, il
ne porte que sur l'attribut `class`, et nos règles ne supportent qu'un seul sélecteur.

Exemple:

```css
.menu-button {
    font-size: 20px;
    width: 200px;
}
```

### Selecteur

Un sélecteur doit vérifier l'expression régulière suivante:

    `.[0-9a-zA-Z\-]+`

### Mots-clés

Voici la liste complète des mots clés supportés:

```css
.all {
    // Font size
    font-size: 20px;
    font-path: fonts.default;
    width: 200px;
    height: 300px;
    background-color: #FFFFFF;
}
```

#### Fonts:

Une font est une image avec des rangées représentant les caractères suivants:

TODO



### Valeur

Les valeurs acceptées peuvent être:
* des flottant avec une unité (PX uniquement pour l'instant)
* une dépendance déclaré dans **Libs**
* une couleur (ex: "#FFFFFF")

### Conflits

Si plusieurs sélecteurs définissent la même propriété, le sélecteur "gagnant"
est celui qui définit la propriété en dernier dans la liste.



## Libs

Ce composant permet de définir les dépendances sur les resources et offre un nommage
pour y accéder.

Par "resource" on inclus:

 * La définition d'une constante
 * Le chemin d'accès à un fichier

Ces définitions sont dans un namespace particulier définit à la déclaration.

De plus on dispose des constructeurs suivant pour une resource:

* `Font` qui permet de charger une fonte. Accepte les paramètres suivants:
  * `path`, un chemin d'accès vers l'image contenant la fonte.
  * `size`, un couple
  * `max-`


```json
fonts {
    default: Font {
        path: "path/to/default-font.png",
        size: (1,2),
    },
    ...
}

cst {
    btn-width: 30,
    btn-height:
}
```
