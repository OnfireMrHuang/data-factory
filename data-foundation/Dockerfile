FROM adoptopenjdk/openjdk13:jdk-13.0.2_8-slim

ENV APP_HOME=/applications/data-foundation

RUN mkdir -p ${APP_HOME}/logs

ADD ./data-foundation-launcher/dist/data-foundation-*.tar.gz ${APP_HOME}/

WORKDIR ${APP_HOME}/bin

EXPOSE 1680

ENTRYPOINT ["busybox", "sh", "application.sh"]
CMD ["start"]