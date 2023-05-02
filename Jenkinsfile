pipeline {
  agent any
  tools { 
        maven 'Maven_3_5_2'  
    }
   stages{
    stage('CompileandRunSonarAnalysis') {
            steps {	
		sh 'mvn clean verify sonar:sonar -Dsonar.projectKey=turnip_registration -Dsonar.organization=simGudim -Dsonar.host.url=https://sonarcloud.io -Dsonar.login=7927f50bd5eafaba6d65a967c3d9c2a317c01c3b'
			}
        } 
  }
}
