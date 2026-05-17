# Matriz de dependencias de ipv8

## Introduccion

`ipv8` no deberia tratar todas las dependencias como si fueran iguales.
Una dependencia puede ser tolerable en laboratorio y peligrosa en produccion.
Puede ser aceptable en la experiencia de escritorio y totalmente inaceptable en identidad o en continuidad operativa.

Por eso `ipv8` necesita una matriz de dependencias.
No para declarar pureza.
Sino para saber que sostiene, que arriesga, que reemplaza y que no deberia normalizar.

## Primera categoria: dependencias fundacionales

Son aquellas sin las cuales el sistema no puede sostener su funcion minima en un perfil dado.
Estas dependencias deben ser pocas, visibles y extremadamente justificadas.

Ejemplos de preguntas para esta categoria.
Sin esta pieza, el nodo puede iniciar.
Sin esta pieza, el nodo puede identificarse.
Sin esta pieza, el nodo puede tomar decisiones locales basicas.
Sin esta pieza, el nodo puede entrar en modo degradado.

Si la respuesta es no, entonces la dependencia es fundacional.
Y por eso mismo debe estar muy controlada.

## Segunda categoria: dependencias operativas

Son dependencias que no crean la esencia del sistema, pero mejoran su capacidad de trabajar con comodidad, escala o alcance.

Aqui pueden entrar herramientas de panel, mecanismos de descubrimiento extendido, empaquetados auxiliares o formas de exportar observabilidad.

Estas dependencias deben existir con reemplazo o con plan de degradacion.

## Tercera categoria: dependencias de experiencia

Son dependencias que mejoran el uso humano, pero no deberian poner en riesgo la continuidad basal del sistema.

Un panel mas elegante.
Una capa visual mas rica.
Una herramienta de traduccion mas amigable.
Una capa de integracion de conveniencia.

Si una dependencia de experiencia se vuelve imprescindible para gobernar el sistema, entonces esta ocupando demasiado poder.

## Cuarta categoria: dependencias de laboratorio

Son dependencias aceptadas para explorar, comparar, experimentar o acelerar descubrimientos.

Estas dependencias no deben ascender silenciosamente a la parte canonica del sistema sin una auditoria clara.
El laboratorio puede tolerar mas riesgo porque su mision no es garantizar estabilidad general, sino aprender sin contaminar el corazon del proyecto.

## Quinta categoria: dependencias de captura

Esta es la categoria mas importante.
Una dependencia de captura es una pieza externa que, aunque hoy parezca comoda, puede terminar concentrando demasiado poder sobre el proyecto.

Algunos signos de captura.
Sin esa pieza no se puede distribuir.
Sin esa pieza no se puede explicar el estado.
Sin esa pieza no se puede reconstruir el sistema.
Sin esa pieza no se puede verificar lo importante.
Sin esa pieza el lenguaje del proyecto se vuelve confuso.

## Matriz de evaluacion

Cada dependencia relevante de `ipv8` deberia evaluarse segun estas dimensiones.

Primera dimension.
Impacto en la existencia minima del sistema.

Segunda dimension.
Impacto en la observabilidad local.

Tercera dimension.
Impacto en la capacidad de migrar.

Cuarta dimension.
Impacto en la claridad semantica.

Quinta dimension.
Facilidad de reemplazo.

Sexta dimension.
Riesgo de captura politica, economica o cognitiva.

Septima dimension.
Compatibilidad con operacion degradada.

## Reglas practicas de la matriz

Si una dependencia afecta identidad, continuidad o decisiones soberanas, debe considerarse de alto escrutinio.
Si una dependencia no tiene plan de reemplazo, su riesgo sube.
Si una dependencia es externa y opaca, su riesgo sube mas.
Si una dependencia mejora mucho la experiencia pero no la continuidad, debe permanecer declarada como no esencial.
Si una dependencia entra primero como laboratorio, debe quedar marcada como temporal hasta que se reevalúe.

## Ejemplos de uso de la matriz

Un dashboard externo podria ser una dependencia de experiencia.
No deberia transformarse en una dependencia fundacional para leer el estado minimo del sistema.

Un mecanismo remoto de bootstrap podria ser una dependencia operativa.
No deberia convertirse en un unico ancla obligatorio de existencia.

Un glosario local no es un lujo documental.
Es una defensa contra la dependencia semantica del entusiasmo variable de cada agente.

## Politica de reduccion de dependencia

No todas las dependencias deben eliminarse ya.
Pero toda dependencia importante deberia tener una politica.

Mantener.
Reducir.
Reemplazar.
Aislar.
Aceptar solo en laboratorio.
Eliminar en una fase futura.

## Cierre

Esta matriz convierte la soberania en una disciplina visible.
Gracias a ella, `ipv8` puede decidir con mas honestidad que dependencias acepta por necesidad, cuales tolera por conveniencia y cuales debe combatir porque amenazan su libertad futura.
