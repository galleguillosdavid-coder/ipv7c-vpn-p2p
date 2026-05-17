# Niveles de soberania por perfil

## Introduccion

No todos los nodos de `ipv8` necesitan el mismo grado de soberania en todas sus capas.
Si el proyecto exige autonomia maxima en todo desde el primer dia, se vuelve inmovil.
Si en cambio no define niveles, la palabra soberania se vacia.

Por eso `ipv8` necesita niveles de soberania por perfil operativo.

## Perfil minimo

Este perfil existe para despliegues simples, personales o de aprendizaje.
Su meta no es dominar el mundo.
Su meta es existir con claridad.

Nivel esperado de soberania.
Alto en lectura local.
Medio en accion local.
Bajo o medio en discovery remoto.
Medio en continuidad parcial.

Lo importante aqui es que el usuario pueda entender su nodo, operarlo localmente y no depender de una gran arquitectura externa para empezar.

## Perfil edge

Este perfil esta pensado para escritorios, laptops, nodos de paso y entornos de campo.
Debe ser flexible, explicable y suficientemente autonomo para entornos imperfectos.

Nivel esperado de soberania.
Alto en lectura.
Alto en accion basica.
Medio o alto en continuidad degradada.
Medio en independencia de pipeline.
Medio en dependencias de descubrimiento.

Aqui la soberania no debe medirse solo por purismo, sino por capacidad real de seguir prestando servicio util en condiciones variables.

## Perfil gateway

Este perfil representa infraestructura de borde, routers, nodos comunitarios o puertas de paso para otros.
Es uno de los perfiles donde mas importa la soberania real.

Nivel esperado de soberania.
Muy alto en continuidad.
Muy alto en politicas locales.
Muy alto en observabilidad local.
Alto en capacidad de degradacion elegante.
Muy bajo en tolerancia a dependencias opacas de captura.

Si un gateway no entiende su propio estado o depende de demasiadas piezas invisibles, se convierte en un punto de fragilidad politica y tecnica.

## Perfil desktop de operador

Este perfil pone enfasis en el control humano de la red, el diagnostico, la lectura de decisiones y la accion local documentada.

Nivel esperado de soberania.
Muy alto en explicabilidad.
Muy alto en lectura y accion.
Medio en continuidad sin interfaz avanzada.
Alto en independencia cognitiva.

Este perfil es clave porque traduce la infraestructura en poder de comprension para el operador.

## Perfil laboratorio soberano

Este perfil acepta mas dependencias experimentales, pero no debe confundirse con falta de disciplina.
Su soberania no consiste en estabilidad total, sino en aislamiento consciente.

Nivel esperado de soberania.
Alto en trazabilidad de experimentos.
Alto en separacion respecto de lo canonico.
Medio en independencia tecnica.
Muy alto en claridad sobre que es temporal y que no.

La soberania del laboratorio es no contaminar al sistema estable con entusiasmo no auditado.

## Perfil comunitario

Este perfil no depende solo del hardware.
Depende del contexto humano.
Barrios, cooperativas, grupos tecnicos, pequeñas organizaciones y redes compartidas pueden necesitar reglas mas legibles y una gobernanza mas distribuida.

Nivel esperado de soberania.
Muy alto en gobernanza legible.
Alto en continuidad comunitaria.
Alto en explicabilidad.
Medio o alto en independencia de plataformas externas.

## Perfil institucional

Este perfil necesita justificar decisiones, registrar evidencias, demostrar politicas y sostener continuidad sin improvisacion excesiva.

Nivel esperado de soberania.
Muy alto en auditabilidad.
Muy alto en trazabilidad.
Alto en control local de observabilidad.
Alto en independencia de proveedores opacos.

## Como usar estos niveles

Los niveles no son decoracion.
Deben servir para decidir.

Que dependencia es tolerable en un perfil y no en otro.
Que modo de operacion puede aceptarse como transitorio.
Que tipo de dashboard necesita cada despliegue.
Que grado de accion local debe estar disponible.
Que tipo de captura externa seria inadmisible en cierto perfil.

## Regla de honestidad

Un perfil no debe declararse mas soberano de lo que realmente es.
Si requiere demasiados anclajes externos para existir, debe admitirlo.
La honestidad en la graduacion de soberania es mejor que la retorica vacia.

## Cierre

Gracias a estos niveles, `ipv8` puede dejar de hablar de soberania como slogan general y empezar a tratarla como una propiedad graduable, medible y coherente con el contexto de cada nodo.
