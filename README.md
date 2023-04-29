# TP1 - Concurrentes - FIUBA

Alumno: Argel Ignacio  
Padrón: 104351

## Requerimientos

- Cada cafetera dispone de N dispensadores independientes que pueden aplicar agua caliente, café molido, cacao o espuma de leche.

- Además la cafetera esta compuesta por:

    - Un contenedor de granos para moler de capacidad G
    - Un contenedor para granos molidos de capacidad M
    - Un contenedor de leche fría de capacidad L
    - Un contenedor de espuma de leche de capacidad E
    - Un contenedor de cacao de capacidad C
    - Un contenedor de agua, donde esta se toma de la red y luego calienta, de capacidad A.

- Cada pedido se representa con una línea en un archivo de texto que indicará las cantidades de café molido, agua caliente, cacao y/o espuma de leche necesarias para preparar la bebida solicitada.

- Las cantidades se "aplicarán" como tiempos de sleep además de descontar del correspondiente contenedor.

- Un solo dispenser a por vez puede tomar ingredientes de cada contenedor, es decir, no es posible por ejemplo que dos disponsers tomen café concurrentemente.

- Cuando el contenedor de cafe molido se agota, el molinillo automático toma una cantidad de granos y los convierte en café molido. Este proceso toma tiempo y no se puede obtener café durante el mismo.

- Análogamente sucede lo mismo con la leche y la espuma, y con el agua caliente.

- El sistema debe minimizar los tiempos de espera de los clientes, maximizando la producción concurrente.

- El sistema debe alertar por consola cuando los contenedores de granos, leche y cacao se encuentran por debajo de X% de capacidad.

- El sistema debe presentar periódicamente estadísticas con los niveles de todos los contenedores, cantidad total de bebidas preparadas y cantidad total de ingredientes consumidos.

## Decisiones de diseño

- El archivo main es un programa que modela una cafetera como la requerida en el enunciado.

- Los dispensers son manejados por una "cola de libres", cuyo acceso esta manejado por un semáforo. Mediante esto se garantiza no iterar los dispensers innecesariamente y que no hayan busy waits.

- Los contenedores son contenidos por un Arc Mutex, que evita que sean tomados por 2 dispensers a la vez.

- En mi implementación, los dispenser sirven siempre en el mismo orden. Este orden esta dado por el iter definido en el enum de contenedores. Se podría optimizar para ir intercalando y ganando tiempo, pero no es un requerimiento y en la realidad al hacer un cafe los ingredientes no son permutables en orden.

- La recarga no necesita de ningún tipo de sincronización, ya que son valores accedidos por y solo por un contenedor. Debido a esto, decidí que los contenedores secundarios para recargar ingredientes, formen parte del mismo struct de cada contenedor.

- El archivo de pedidos es `orders.txt` con el siguiente formato:

    30,12,1,1
    1,3,8,1
    1,9,1,1
    25,9,1,1

Donde la primera columna es cafe, la segunda agua, la tercera espuma y la cuarta cacao (según el iter del enum).

- En cuanto a los reportes, los mismo son hechos cada 3 cafes completados, y muestran la cantidad de cada contenedor y la cantidad de ingredientes gastados por la cafetera hasta el momento.

- Por decision de diseño, los reportes son activados cada cierto tiempo. Esto desencadena una reacción en los contenedores que, cuando terminan lo que están haciendo, imprimen sus estadísticas al momento.