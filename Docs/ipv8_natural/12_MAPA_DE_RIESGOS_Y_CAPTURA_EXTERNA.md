# Mapa de riesgos y captura externa

## Introduccion

Todo proyecto que habla de independencia y soberania deberia estudiar no solo sus metas, sino tambien las formas en que podria perderlas.

La captura externa no siempre llega como censura directa.
A veces llega como comodidad.
A veces llega como dependencia amable.
A veces llega como una herramienta tan util que termina volviendose inamovible.

## Riesgo uno: captura por pipeline

Cuando la cadena principal de build, validacion o distribucion depende excesivamente de una plataforma externa, el proyecto puede perder autonomia sin notarlo.

El riesgo no es usar herramientas externas.
El riesgo es no tener una ruta local y verificable cuando esas herramientas cambian reglas, disponibilidad o costo.

## Riesgo dos: captura por observabilidad

Si la verdad operativa solo puede verse a traves de un servicio remoto, un panel externo o un proveedor tercero, el operador local pierde poder.

La captura aqui ocurre cuando la red funciona, pero el entendimiento de la red ya no pertenece al operador.

## Riesgo tres: captura por semantica

Un proyecto tambien puede ser capturado por confusion conceptual.
Si cada agente redefine palabras importantes, la arquitectura se vuelve maleable al caos.

La falta de glosario, de contratos y de decisiones registradas no parece un problema politico al principio.
Pero termina siendolo, porque destruye la capacidad del proyecto de sostener su propia identidad intelectual.

## Riesgo cuatro: captura por bootstrap unico

Si el sistema depende de una sola forma externa de descubrir el mundo, cualquier problema en ese punto lo vuelve fragil.

`ipv8` debe tratar con cuidado toda forma de ancla unica.
Repositorio unico.
Seed unica.
Servicio unico.
Identidad unica centralizada.

## Riesgo cinco: captura por interfaz

A veces una interfaz tan comoda concentra demasiado poder.
Si el usuario ya no sabe operar el sistema sin cierta pantalla, cierto panel o cierta capa visual, entonces la UX empezo a secuestrar a la operacion.

Una buena interfaz debe empoderar, no volver imposible el funcionamiento sin ella.

## Riesgo seis: captura por automatizacion

La automatizacion mal gobernada puede desplazar la soberania humana.

Si una capa automatica empieza a tomar demasiadas decisiones sin explicabilidad, sin trazabilidad o sin limites, el proyecto puede terminar obedeciendo a su propia comodidad automatizada.

## Riesgo siete: captura por memoria centralizada

Si el conocimiento operativo real queda atrapado en una maquina, en una sesion o en un agente particular, el sistema se vuelve dependiente de ese centro.

La documentacion natural y la memoria operativa portable son defensas directas contra esta forma de captura.

## Riesgo ocho: captura por dependencia de reputacion opaca

Si la reputacion se convierte en una caja negra, la red puede empezar a tomar decisiones con gran impacto politico sin posibilidad de justificacion seria.

Eso vuelve a `ipv8` vulnerable a decisiones arbitrarias, sesgos y disputas imposibles de resolver con evidencia comprensible.

## Riesgo nueve: captura por migracion imposible

Un sistema tambien pierde soberania cuando ya no puede cambiar sin romperse por completo.

Si migrar desde una version, un modulo o una dependencia se vuelve traumatico, esa pieza empieza a capturar al proyecto por inercia.

## Riesgo diez: captura por grandilocuencia

Existe un riesgo menos tecnico pero muy real.
Cuando un proyecto se enamora de su propia narrativa, puede dejar de ver donde esta realmente atado.

La grandilocuencia puede esconder dependencia.
Puede esconder fragilidad.
Puede esconder falta de camino local.
Puede esconder opacidad.

## Politica de respuesta ante riesgos

Cada riesgo importante deberia tener una respuesta pensada.

Hacer visible.
Medir.
Reducir.
Aislar.
Reemplazar.
Mantener solo en laboratorio.
Prohibir en perfiles de alta soberania.

## Como usar este mapa

Este documento no es para generar miedo.
Es para conservar lucidez.

Antes de aceptar una nueva pieza, una nueva comodidad o una nueva integracion, `ipv8` deberia preguntarse.
Esto nos da poder.
O nos lo quita lentamente.

## Cierre

La captura externa rara vez entra anunciandose como captura.
Suele entrar como solucion facil.
Por eso `ipv8` necesita este mapa: para recordar que la independencia no se pierde solo por ataque, sino tambien por acumulacion de concesiones no pensadas.
